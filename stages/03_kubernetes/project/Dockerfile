FROM python:3.8.6-alpine3.12


COPY ./app /usr/src/project
WORKDIR /usr/src/project
RUN apk add build-base && apk add postgresql-dev && pip install -r ./requirements.txt


EXPOSE 8000

ENTRYPOINT ["python", "/usr/src/project/app.py"]
