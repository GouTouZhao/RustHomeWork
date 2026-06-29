@echo off
REM Create namespace
kubectl create namespace forum

REM Deploy backend services
kubectl apply -f .\ForumBackend\deployments\deploy\ -n forum

REM Deploy frontend services
kubectl apply -f .\ForumFrontend\deployments\deploy\ -n forum

echo.
echo [SUCCESS] K8s deployment triggered! 
echo Run "kubectl get pods -n forum" to check status.
echo Frontend is accessible via NodePort 30080: http://^<Your-Node-IP^>:30080
echo.
pause
