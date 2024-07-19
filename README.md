# PulldownCMarkLib

This is a simple DLL library to use **pulldown-cmark** from any win32 application as CDECL.

# How to build

- 64bits: 
```
rustup default stable-x86_64-pc-windows-msvc
cargo build --target=x86_64-pc-windows-msvc --release
```
- 32bits: 
```
rustup default stable-i686-pc-windows-msvc
cargo build --target=i686-pc-windows-msvc --release
```


For instance the **pascal** directory contains a library to use with Delphi.

## Usage from Delphi

- Put the `pulldowncmarklib.dll` in your application directory or make it available in ENVIRONMENT.
- Add to `uses`and just use as follows:

```pascal
uses PulldownCMark;
...

Memo2.Text := StringToMarkdown(Memo1.Text, [ moEnableTables,
    moEnableFootnotes,
    moEnableStrikethrough,
    moEnableTasklists,
    moEnableSmartPunctuation,
    moEnableHeadingAttributes,
    moEnableYamlStyleMetadataBlocks,
    moEnablePlusesDelimitedMetadataBlocks,
    moEnableOldFootnotes,
    moEnableMath,
    moEnableGFM])); 
``` 
## Exports to use on any other program.
The DLL exports two functions, the free_string must be called after to release the allocated memory for the result.
```
PAnsiChar strtomarkdown(PAnsiChar input, uint32 options);

void free_string(PAnsiChar ptr);
```

The `options` parameter is a bit flag that can be constructed using the following values:

```
ENABLE_TABLES                    = 1 << 1
ENABLE_FOOTNOTES                 = 1 << 2
ENABLE_STRIKETHROUGH             = 1 << 3
ENABLE_TASKLISTS                 = 1 << 4
ENABLE_SMART_PUNCTUATION         = 1 << 5
ENABLE_HEADING_ATTRIBUTES        = 1 << 6
ENABLE_YAML_STYLE_METADATA_BLOCKS = 1 << 7
ENABLE_PLUSES_DELIMITED_METADATA_BLOCKS = 1 << 8
ENABLE_OLD_FOOTNOTES             = (1 << 9) | (1 << 2)
ENABLE_MATH                      = 1 << 10
ENABLE_GFM                       = 1 << 11
```

### C# Example

```cs
using System;
using System.Runtime.InteropServices;
using System.Text;

class PulldownCMark
{
    const string DllName = "pulldowncmarklib.dll";

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    private static extern IntPtr strtomarkdown(IntPtr input, uint options);

    [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
    private static extern void free_string(IntPtr ptr);

    public static string StringToMarkdown(string input, uint options)
    {
        var inputBytes = Encoding.UTF8.GetBytes(input);
        var inputPtr = Marshal.AllocHGlobal(inputBytes.Length + 1);
        Marshal.Copy(inputBytes, 0, inputPtr, inputBytes.Length);
        Marshal.WriteByte(inputPtr, inputBytes.Length, 0);

        var resultPtr = strtomarkdown(inputPtr, options);
        var result = Marshal.PtrToStringAnsi(resultPtr);

        Marshal.FreeHGlobal(inputPtr);
        free_string(resultPtr);

        return result;
    }

    public static void Main()
    {
        string markdown = "# Hello, world!\nThis is a **test**.\n";
        uint options = (1 << 1) | (1 << 3); // ENABLE_TABLES | ENABLE_STRIKETHROUGH
        string html = StringToMarkdown(markdown, options);
        Console.WriteLine(html);
    }
}
```

### C++ Example

```cpp
#include <iostream>
#include <string>
#include <windows.h>

typedef const char* (*StrToMarkdownFunc)(const char*, uint32_t);
typedef void (*FreeStringFunc)(const char*);

int main() {
    HMODULE dll = LoadLibraryA("pulldowncmarklib.dll");
    if (!dll) {
        std::cerr << "Failed to load DLL" << std::endl;
        return 1;
    }

    auto strtomarkdown = (StrToMarkdownFunc)GetProcAddress(dll, "strtomarkdown");
    auto free_string = (FreeStringFunc)GetProcAddress(dll, "free_string");

    if (!strtomarkdown || !free_string) {
        std::cerr << "Failed to load functions" << std::endl;
        FreeLibrary(dll);
        return 1;
    }

    const char* markdown = "# Hello, world!\nThis is a **test**.\n";
    uint32_t options = (1 << 1) | (1 << 3); // ENABLE_TABLES | ENABLE_STRIKETHROUGH

    const char* html = strtomarkdown(markdown, options);
    std::cout << html << std::endl;

    free_string(html);
    FreeLibrary(dll);

    return 0;
}
```

### Zig example

```zig
const std = @import("std");
const windows = std.os.windows;

extern "pulldowncmarklib" fn strtomarkdown(input: [*:0]const u8, options: u32) [*:0]const u8;
extern "pulldowncmarklib" fn free_string(ptr: [*:0]const u8) void;

pub fn main() !void {
    const markdown = "# Hello, world!\nThis is a **test**.\n";
    const options: u32 = (1 << 1) | (1 << 3); // ENABLE_TABLES | ENABLE_STRIKETHROUGH

    const result = strtomarkdown(markdown, options);
    defer free_string(result);

    const stdout = std.io.getStdOut().writer();
    try stdout.print("{s}\n", .{result});
}
```

Note: Make sure to have the pulldowncmarklib.dll avaialble for the executables. 

Remember that the Rust string handling is **UTF-8 compliant**, so in languages that use UTF-16 (like C# for strings), you may need to perform proper encoding conversions as shown in the C# example.


## DISCLAIMER

The examples are just guides, I didn't test them.