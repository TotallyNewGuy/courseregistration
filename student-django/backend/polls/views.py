from django.shortcuts import render
from django.db import connection

# Create your views here.
def test_backend(request):
    context = {"data": "hello from backend"}
    return render(request, "test_page.html", context)

def test_db(request):
    cursor = connection.cursor()
    cursor.execute(f"select * from `student`")
    rows = cursor.fetchall()
    context = {"data": rows}
    return render(request, "test_page.html", context)