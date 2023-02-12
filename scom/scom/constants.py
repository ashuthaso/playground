import configparser

parser = configparser.ConfigParser()
parser.read("scom.conf")

HEADER_SIZE = parser.get("scom_config", "header_size")
PORT = parser.get("scom_config", "port")
DISCONNECT_MSG = parser.get("scom_config", "disconnect_msg")
