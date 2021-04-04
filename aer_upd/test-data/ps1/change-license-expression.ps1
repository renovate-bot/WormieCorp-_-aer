param($data)

Write-Warning "This script changes the license expression, this should be respected!"
$data.license.expr = "Apache-2.0"
