import urllib.request

while True:
    contents = urllib.request.urlopen("http://localhost:8080/hello").read()
    print(contents)
