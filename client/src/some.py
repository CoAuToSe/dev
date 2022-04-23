import requests

url = 'https://queue-times.com/fr/parks/4/rides/2'
r = requests.get(url)
print(r.text)