import time
import random

buf2 = [59, 25, 89, 18, 63, 60, 32, 28, 95, 74, 11, 0] #given to us and is what we need to get the final answer.
seeded_generated_string = []
user_input = []
password_string = []
counter = 0
counter2 = 0

#1715622095
current_time = 1715622095
#current_time = time.time(1715622095) #gets current time of the system
random.seed(int(current_time)) #setting seed to system time 
print("Current Time: ", hex(current_time))


while counter < 8 :
    random_number = random.randint(0, 2147483647) #generates the number we need that will be seeded with time.
    generated_value = random_number % 94 + 33
    print("Random Number: " , random_number)
    seeded_generated_string.append(generated_value)
    counter = counter + 1

while counter2 < 12 :
    
    xor_value = seeded_generated_string[counter2 % 8] ^ buf2[counter2]
    user_input.append()
    counter2 = counter2 + 1

    user_input_hex = [hex(value) for value in user_input]

##create function to analyze the get the seeded generated string.
### Need to get srand c equivalent and rand() c equivalent
## create function to analyze (requires buf2)
