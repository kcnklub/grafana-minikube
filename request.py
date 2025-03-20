import urllib.request

while True:
    contents = urllib.request.urlopen("http://localhost:9101/hello").read()
    print(contents)
