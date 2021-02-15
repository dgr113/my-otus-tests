Собираем и разворачиваем приложение auth-app:
```shell
helm install auth-app ./auth_app/auth-chart;
```
<br>

Собираем и разворачиваем приложение sample-app:
```shell
helm install sample-app ./sample_app/sample-chart;
```
<br>

Добавляем репозиторий, устанавливаем ambassador-aes и применяем манифесты
```shell
helm repo add datawire https://getambassador.io;
helm install aes datawire/ambassador -f ./ambassador/values.yaml;
kubectl apply -f ./ambassador/kube_conf;
```
<br>
