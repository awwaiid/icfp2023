# ICFP CONTEST 2023!!

This year we didn't get much planning in ahead of time ... so going remote-async!

Create solutions with this structure:

  solutions/solution-{problem number}-score-{score}-strategy-{ bot name }.json

then we can submit best scores. We should make a script that submits the best ones?

submission token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1aWQiOiI2NGE4ODlkODhjNGVmY2E4Y2IwOWRjNzciLCJpYXQiOjE2ODg3NjY5MzYsImV4cCI6MTY5ODc2NjkzNn0.c-jX9nkHvFkgUpLTUxZyDw3GyiatxHx2sTS681Oxevw


To submit solutions you can set the above token to the environment variable API_KEY and run ./bin/submit_the_thing <list of solution files>
The response will be the status code and, if successful, the submission id.

