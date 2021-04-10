## Установка:
Добавляем репозиторий с `Prometheus`
```shell
helm repo add prometheus-community https://prometheus-community.github.io/helm-charts;
helm repo update;
```
Устанавливаем чарт с `Prometheus`
```shell
helm install prom prometheus-community/kube-prometheus-stack -f ./prometheus-chart/prometheus.yaml --atomic;
```
Собираем и разворачиваем приложение (`namespace` не важен):
```shell
helm install users-app ./users-app-chart;
```


## Пример проброса портов (для тестирования\дебага)
```shell
kubectl port-forward service/prom-grafana 9000:80;
# Данные для входа в админку (https://github.com/prometheus-community/helm-charts/blob/main/charts/kube-prometheus-stack/values.yaml)
# login: admin
# password: prom-operator
```


## API tests:
[Postman](./tests/88_prometheus.postman_collection.json)


## LOAD tests:
```shell
k6 run -e BASE_URL=http://172.17.28.216 ./tests/k6-load-test.js;
```

## Dashboards:
[Grafana](./graphana_dashboard.json)
