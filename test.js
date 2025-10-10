// Test cases for travel calculation functions

function parseDate(str) {
  const [y, m, d] = str.split('-').map(Number);
  return new Date(y, m - 1, d);
}

function daysBetween(start, end) {
  return Math.floor((end - start) / (1000 * 60 * 60 * 24)) + 1;
}

function parseTravelPeriods(input) {
  return input.split('\n').map(line => {
    const [start, end] = line.trim().split(' to ').map(parseDate);
    return { start, end, days: daysBetween(start, end) };
  }).filter(p => p.start && p.end);
}

// Test cases
const tests = [
  {
    name: "Same day",
    start: "2023-01-01",
    end: "2023-01-01", 
    expected: 1
  },
  {
    name: "7-day trip",
    start: "2023-01-01",
    end: "2023-01-07",
    expected: 7
  },
  {
    name: "31-day January",
    start: "2023-01-01", 
    end: "2023-01-31",
    expected: 31
  },
  {
    name: "Leap year Feb",
    start: "2024-02-01",
    end: "2024-02-29", 
    expected: 29
  },
  {
    name: "Non-leap year Feb",
    start: "2023-02-01",
    end: "2023-02-28",
    expected: 28
  }
];

const realWorldTest = {
  name: "Real world example - Arrival 01-11-2021",
  arrivalDate: "2021-11-01",
  travelDates: `2023-11-24 to 2024-03-13
2022-10-28 to 2023-02-13
2024-12-09 to 2025-03-29
2025-10-13 to 2025-10-19
2025-10-29 to 2025-11-04`,
  expectedTotalDays: 345 // 111 + 109 + 111 + 7 + 7
};

// Run basic tests
console.log("Running travel calculation tests...\n");

let passed = 0;
let failed = 0;

tests.forEach(test => {
  const start = parseDate(test.start);
  const end = parseDate(test.end);
  const result = daysBetween(start, end);
  
  if (result === test.expected) {
    console.log(`✅ ${test.name}: ${result} days`);
    passed++;
  } else {
    console.log(`❌ ${test.name}: Expected ${test.expected}, got ${result}`);
    failed++;
  }
});

// Run real world test
console.log("\nReal world test:");
const travelPeriods = parseTravelPeriods(realWorldTest.travelDates);
let totalDays = 0;
travelPeriods.forEach(p => {
  console.log(`  ${p.start.toDateString()} to ${p.end.toDateString()}: ${p.days} days`);
  totalDays += p.days;
});

console.log(`\nTotal days traveled: ${totalDays}`);
console.log(`Expected: ${realWorldTest.expectedTotalDays}`);
console.log(`How many more days left to travel? ${Math.max(0, 450 - totalDays)} days`);

if (totalDays === realWorldTest.expectedTotalDays) {
  console.log(`✅ Real world test passed`);
  passed++;
} else {
  console.log(`❌ Real world test failed`);
  failed++;
}

console.log(`\nResults: ${passed} passed, ${failed} failed`);