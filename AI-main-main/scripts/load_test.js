import { check, sleep } from 'k6';
import http from 'k6/http';

export let options = {
  stages: [
    { duration: '2m', target: 100 }, // Ramp up to 100 users
    { duration: '5m', target: 100 }, // Stay at 100 users
    { duration: '2m', target: 0 },   // Ramp down to 0 users
  ],
};

export default function () {
  let res = http.get('http://your-service-url/health');
  check(res, {
    'is status 200': (r) => r.status === 200,
  });
  sleep(1);
} 