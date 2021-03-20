param($data)

Write-Output "This script changes the license url, this should be respected!"
$data.license.url = "https://github.com/AdmiringWorm/chocolatey-packages/blob/master/LICENSE.txt"
