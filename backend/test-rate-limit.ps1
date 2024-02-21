1..150 | ForEach-Object {
    curl http://127.0.0.1:8000/
    Write-Output "Request $_"
}
