from os import environ

DEBUG = environ.get("DEBUG", True)
SECRET_KEY = environ.get("SECRET_KEY", "debug")
