@echo off
REM 创建命名空间
kubectl create namespace forum

REM 部署后端所有的服务
kubectl apply -f .\ForumBackend\deployments\deploy\ -n forum

REM 部署前端服务
kubectl apply -f .\ForumFrontend\deployments\deploy\ -n forum

echo.
echo K8s 部署已触发！你可以通过 kubectl get pods -n forum 查看启动状态。
echo 前端入口可通过 NodePort 30080 访问: http://^<你的节点IP^>:30080
echo.
pause
