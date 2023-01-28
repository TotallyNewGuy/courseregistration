from django.shortcuts import render

# Create your views here.
def send(request, email):
    if request.method == 'POST':
        return render(request, "send.html", {"data": email})
    else :
        text="it's not a POST request"
        return render(request, "send.html", {"data": text})

def test(request):
    text = "hello from backend"
    return render(request, "send.html", {"data": text})