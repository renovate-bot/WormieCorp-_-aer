$ErrorActionPreference = 'Stop';
$InformationPreference = 'Continue';
$VerbosePreference = 'Continue';
$DebugPreference = 'Continue';
{extra_code} # Any code that is necessary on different platfroms.
$data = ("{script_data}" | ConvertFrom-Json -AsHashtable)

[int]$exitCode = 0;

try {{
    {script_path} $data;
    [int]$exitCode = $LASTEXITCODE;
}}
catch {{
    Write-Error $_;
    if ($LASTEXITCODE -eq 0) {{
        [int]$exitCode = 1;
    }}
}}

Write-Host "## AER-SCRIPT-RUNNER:START ##";
Write-Host ($data | ConvertTo-Json);
Write-Host "## AER-SCRIPT-RUNNER:END ##";
if ($exitCode -ne 0) {{
    throw "Non-Zero exit code: $exitCode";
}}
