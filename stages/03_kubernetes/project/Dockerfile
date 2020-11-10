FROM python:3.8.6-alpine3.12


WORKDIR /usr/src/
COPY ./requirements.txt ./app/requirements.txt
RUN apk add build-base && apk add postgresql-dev && pip install -r ./app/requirements.txt

COPY ./app /usr/src/app


EXPOSE 8000
ENTRYPOINT ["python", "-m", "app"]
