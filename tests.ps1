
cargo build
if(-not $?){
    Write-Error "Build failed"
    exit 1
}

$tests = Get-ChildItem -Path "$PSScriptRoot/tests/" -Name

foreach($test in $tests)
{
    cargo run --bin zmake -- "@$PSScriptRoot/tests/$test/argfile" -C "$PSScriptRoot/tests/$test"
    if(-not $?){
        Write-Error "Test failed"
        exit 1
    }
}

exit 0
