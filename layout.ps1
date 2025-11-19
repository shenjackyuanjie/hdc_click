# 1. 执行 dump 命令并将输出结果保存到变量 $output 中
Write-Host "正在生成 UI 结构..." -ForegroundColor Yellow
$output = hdc shell uitest dumpLayout

# 打印设备返回的原始信息，让你看到发生了什么
Write-Host "设备返回: $output" -ForegroundColor Gray

# 2. 使用正则从输出中提取文件路径 (匹配 saved to: 后面的内容)
if ($output -match "saved to:(/data/local/tmp/.*\.json)") {
    # $Matches[1] 就是提取到的路径，例如 /data/local/tmp/layout_123.json
    $remotePath = $Matches[1].Trim()
    
    Write-Host "已定位文件路径: $remotePath" -ForegroundColor Green
    
    # 3. 拉取文件并重命名为 layout.json
    hdc file recv $remotePath ./layout.json
    
    if (Test-Path ./layout.json) {
        Write-Host "🎉 成功！文件已保存在当前目录: layout.json" -ForegroundColor Green
        
        # 4. (可选) 删除设备上的临时文件，节省空间
        hdc shell rm $remotePath
    } else {
        Write-Host "❌ 拉取失败，请检查连接。" -ForegroundColor Red
    }
} else {
    Write-Host "❌ 未能从输出中解析出文件路径。可能是 dump 失败了。" -ForegroundColor Red
}