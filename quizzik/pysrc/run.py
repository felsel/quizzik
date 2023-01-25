import os

import quizziklib

# os.chdir("../")
# print(os.listdir())
sum = quizziklib.sum_as_string(2, 2)
print(sum)

db_user = quizziklib.connect_to_db()
print(db_user)

count_dict = quizziklib.word_counting.count_word("hello my name is joelly")
print(count_dict)
