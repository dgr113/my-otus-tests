FROM python:3.8.6-alpine3.12


COPY ../project /usr/src/project
WORKDIR /usr/src/project
RUN pip install -r ./requirements.txt


EXPOSE 8000

ENTRYPOINT ["python", "/usr/src/project/app.py"]
