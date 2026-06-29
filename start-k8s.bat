@echo off
REM Create namespace (ignore error if exists)
kubectl create namespace forum

REM Create ConfigMap from local config files
kubectl create configmap forum-config --from-file=.\ForumBackend\config\ -n forum

REM Create Secret for OSS (ignore error if exists or update it if needed, simple create for now)
kubectl create secret generic forum-secret --from-literal=OSS_ACCESSKEYID=replace_me --from-literal=OSS_ACCESSKEYSECRET=replace_me --from-literal=OSS_ROLEARN=replace_me -n forum

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
