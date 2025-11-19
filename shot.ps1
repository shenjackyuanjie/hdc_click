# ===================================================================
# PowerShell 脚本: 自动化鸿蒙设备截图并拉取到本地 (v2 - 修正版)
#
# 更新日志:
# - 根据 hdc 报错信息，将截图格式从 .png 更改为 .jpeg。
# - 优化了错误捕获机制，能够显示 hdc 返回的具体错误。
# ===================================================================

# 1. 自动生成唯一的文件名 (格式: YYYYMMDD_HHMMSS.jpeg)
$timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
$fileName = "$($timestamp).jpeg" # <--- 已将后缀从 .png 修改为 .jpeg

# 2. 定义设备上的临时路径和本地保存路径
$devicePath = "/data/local/tmp/$fileName"
$localPath = ".\" # "." 代表当前目录

# 打印执行信息
Write-Host "======================================================" -ForegroundColor Green
Write-Host " 开始执行鸿蒙设备截图..."
Write-Host "======================================================"
Write-Host "设备上的文件路径: $($devicePath)"
Write-Host "将要保存到本地: $($PSScriptRoot)\$($fileName)"
Write-Host ""

# 3. 第一步: 在设备上执行截图命令，并捕获所有输出
Write-Host "步骤 1/3: 正在设备上截图..." -ForegroundColor Yellow
# 使用 2>&1 将错误流重定向到标准输出流，以便捕获
$screenshotResult = hdc shell snapshot_display -f $devicePath 2>&1

# 检查上一个命令的输出是否包含错误信息或退出码不为0
if (($LASTEXITCODE -ne 0) -or ($screenshotResult -match "error|invalid|fail")) {
    Write-Host "错误: 截图失败！请检查 hdc 环境和设备连接。" -ForegroundColor Red
    Write-Host "HDC 返回的详细信息: $($screenshotResult)" -ForegroundColor Yellow
    exit 1
}
Write-Host "截图成功。" -ForegroundColor Green
Write-Host ""

# 4. 第二步: 将截图文件从设备拉取到本地
Write-Host "步骤 2/3: 正在将文件拉取到本地..." -ForegroundColor Yellow
$recvResult = hdc file recv $devicePath $localPath 2>&1

# 检查文件是否成功拉取
if (($LASTEXITCODE -ne 0) -or ($recvResult -match "error|invalid|fail")) {
    Write-Host "错误: 拉取文件失败！" -ForegroundColor Red
    Write-Host "HDC 返回的详细信息: $($recvResult)" -ForegroundColor Yellow
    exit 1
}

# 再次确认本地文件是否存在
if (-not (Test-Path "$($localPath)\$($fileName)")) {
    Write-Host "错误: 文件未成功传输到本地目录。" -ForegroundColor Red
    exit 1
}
Write-Host "文件拉取成功。" -ForegroundColor Green
Write-Host ""

# 5. 第三步: 清理设备上的临时文件
Write-Host "步骤 3/3: 正在清理设备上的临时文件..." -ForegroundColor Yellow
hdc shell rm $devicePath

if ($LASTEXITCODE -ne 0) {
    Write-Host "警告: 未能删除设备上的临时文件，可手动清理。" -ForegroundColor DarkYellow
} else {
    Write-Host "设备临时文件已清理。" -ForegroundColor Green
}
Write-Host ""

# 6. 完成提示
Write-Host "======================================================" -ForegroundColor Green
Write-Host "🎉 操作成功！截图已保存为: $($fileName)"
Write-Host "======================================================"
