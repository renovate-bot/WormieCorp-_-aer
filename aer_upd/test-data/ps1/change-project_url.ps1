param([hashtable]$data)

Write-Information "This script changes the project_url, this should be respected!"
$data.project_url = "https://github.com/WormieCorp/aer"
