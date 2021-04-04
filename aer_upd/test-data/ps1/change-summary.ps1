param([hashtable]$data)

Write-Verbose "This script changes the license expression, this should be respected!"
$data.summary = "The summary was changed to something else"
