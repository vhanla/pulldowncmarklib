unit PulldownCMark;

interface


{ Pulldown-Mark library}
const
  PULLDOWNCMARKDLL = 'pulldowncmarklib.dll';
type
// https://github.com/pulldown-cmark/pulldown-cmark/blob/8b060d5befc253914199d73c301283654ae0ca85/pulldown-cmark/src/lib.rs#L366
  TPulldownCMarkOption = (
    moEnableTables,
    moEnableFootnotes,
    moEnableStrikethrough,
    moEnableTasklists,
    moEnableSmartPunctuation,
    moEnableHeadingAttributes,
    moEnableYamlStyleMetadataBlocks,
    moEnablePlusesDelimitedMetadataBlocks,
    moEnableOldFootnotes,
    moEnableMath,
    moEnableGFM
  );
  TPulldownCMarkOptions = set of TPulldownCMarkOption;
  TPulldownCMarkHelper = record
  public
    class function OptionsToU32(const Options: TPulldownCMarkOptions): UInt32; static;
  end;

  function strtomarkdown(input: PAnsiChar; options: uint32): PAnsiChar; cdecl;
    external PULLDOWNCMARKDLL;
  procedure free_string(ptr: PAnsiChar); cdecl;
    external PULLDOWNCMARKDLL;

  function StringToMarkdown(const Input: string; const Options: TPulldownCMarkOptions): string;
implementation

{ TPulldownMarkHelper }

class function TPulldownCMarkHelper.OptionsToU32(
  const Options: TPulldownCMarkOptions): UInt32;
const
  OptionsFlags: array[TPulldownCMarkOption] of UInt32 = (
    1 shl 1,
    1 shl 2,
    1 shl 3,
    1 shl 4,
    1 shl 5,
    1 shl 6,
    1 shl 7,
    1 shl 8,
    (1 shl 9) or (1 shl 2),
    1 shl 10,
    1 shl 11
  );
var
  Option: TPulldownCMarkOption;
begin
  Result := 0;
  for Option := Low(TPulldownCMarkOption) to High(TPulldownCMarkOption) do
    if Option in Options then
      Result := Result or OptionsFlags[Option];
end;

function StringToMarkdown(const Input: string; const Options: TPulldownCMarkOptions): string;
var
  inputCStr, outputCStr: PAnsiChar;
begin
  inputCStr := PAnsiChar(UTF8Encode(Input));
    // Call Rust function
  outputCStr := strtomarkdown(inputCStr, TPulldownCMarkHelper.OptionsToU32(Options));
  Result := WideString(UTF8Decode(outputCStr));
  free_string(outputCStr);
end;

end.
