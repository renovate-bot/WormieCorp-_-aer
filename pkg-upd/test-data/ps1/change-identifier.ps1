param([hashtable]$data)

Write-Warning "This script tries to change the identifier, which is not respected!"
$data.id = "changed"
