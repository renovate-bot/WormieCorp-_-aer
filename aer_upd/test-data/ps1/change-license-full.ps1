param($data)

Write-Warning "This script changes the license expression and url, this should be respected!"
$data.license = @{
    expr = "Apache-2.0"
    url  = "https://github.com/AdmiringWorm/chocolatey-packages/blob/master/LICENSE.txt"
}
