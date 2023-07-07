#!/usr/bin/env ts-node

let fs = require("fs");
let data = fs.readFileSync(0, 'utf-8');
let problem = JSON.parse(data);

let musicianCount = problem.musicians.length;

let stageMinX = problem.stage_bottom_left[0];
let stageMinY = problem.stage_bottom_left[1];

let stageMaxX = stageMinX + problem.stage_width;
let stageMaxY = stageMinY + problem.stage_height;


let placements = [];
iter: for(let y = stageMinY + 10; y < stageMaxY - 10; y += 10) {
  for(let x = stageMinX + 10; x < stageMaxX - 10; x += 10) {
    if (placements.length === musicianCount) {
      break iter;
    }

    placements.push({ x, y });
  }
}

process.stdout.write(JSON.stringify({placements}) + "\n");

