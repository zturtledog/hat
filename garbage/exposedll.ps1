param($p1)
$mypath = $MyInvocation.MyCommand.Path
cd "C:\Program Files (x86)\Microsoft Visual Studio\2019\BuildTools\VC\Tools\MSVC\14.29.30133\bin\Hostx64\x86"
./link.exe /export D:\rusty_programs\hat\raylib.dll
$mypath = Split-Path -Path $mypath
cd $mypath