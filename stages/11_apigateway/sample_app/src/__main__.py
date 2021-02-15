# coding: utf-8

from .app_factory import create_app  # type: ignore

app = create_app( __name__ )




if __name__ == "__main__":
    app.run(host='0.0.0.0', port='80', debug=True)
