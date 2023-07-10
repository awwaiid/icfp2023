# ICFP CONTEST 2023!!

This year we didn't get much planning in ahead of time ... so going remote-async!

## General idea

We get a collection of problems and build out a collection of solutions. We submit them to get an overall score. This lends itself to trying to maximize the score for each problem using any algorithm or process available.

Create solutions with this structure:

    solutions/solution-{problem number}-score-{score}-strategy-{ bot name }.json

then we can submit best scores. We should make a script that submits the best ones? A script that gets and fills in the score? Scoring is async.

## Submit

submission token: eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1aWQiOiI2NGE4ODlkODhjNGVmY2E4Y2IwOWRjNzciLCJpYXQiOjE2ODg3NjY5MzYsImV4cCI6MTY5ODc2NjkzNn0.c-jX9nkHvFkgUpLTUxZyDw3GyiatxHx2sTS681Oxevw

To submit solutions you can set the above token to the environment variable API_KEY and run ./bin/submit_the_thing <list of solution files>
The response will be the status code and, if successful, the submission id.

## Things to run

```sh
bin/submit_the_thing solutions/solution-*trivial.json

<problems/problem-1.json| bin/solve

diff -u <(jq . solution-7-score-unknown-strategy-random.json) <(jq . solution-7-score-unknown-strategy-trivial.json )

time (for i in {1..90} ; do <problems/problem-$i.json| bin/solve > solutions/solution-$i-score-unknown-strategy-random.json ; done)

jq . solution-52-score-unknown-strategy-trivial.json
```

## TECHNOLOGY

This year we used:
* Rust
* ChatGPT
* TypeScript
* Bash
* Discord
* Git :)

## IDEAS

- optimization problem, so gotta get a good evaluator for sure and then ........ GA?!
- manual with visualizer
- some sort of spring model
- collection of heuristics of trying to please individual audience members with strong preferences
- these are all floating point locations, but we could pretend that the stage is broken into a grid of positions. That might eliminate some good solutions but it might make other solutions easier to find
- it might be interesting to calculate the fake-max-score, like for each audience member all of their positive preferences are located at the closest distance on the stage (all on top of each other, so not like real life), and all of their negative preferences are either far away or better yet blocked. Then if we can get the scores of our submissions we could find the problems with the largest potential improvement and prioritize those
