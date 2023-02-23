# Course Registration System  
This is a briefly introduction of this project.  
I drawn some diagrams to explain my results.  

## A. Four services in two three languages

![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/services.png)

## B. Services logic: user call student service to send emails
![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/services_logic.png)

## C. Detail of implementation of message queue  
### 1. Publish

![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/mq_publish.png)  

### 2. Subscribe

![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/mq_subscribe.png)  

## 3. running in django

![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/run_in_django.png)  

## D. Detail of implementation of registry  
### 1. Logic
![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/registry-logic.png)  

### 2. Raft algorithm
![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/raft%20logic.png)  

### 3. Fault tolerance
![image](https://github.com/TotallyNewGuy/courseregistration/blob/main/picture/raft-consensus.png)  
