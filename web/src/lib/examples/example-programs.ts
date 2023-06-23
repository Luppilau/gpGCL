export const coin_example = `x := 0;
while (x < 0.5) {
  x := uniform(0,1)
}`;

export const bounded_retransmission = `fail := 0;
sent := 0;
while (sent < 8000000 && fail < 10) {
  { fail := 0; sent := sent + 1} [0.999] {fail := fail + 1}
}`;

export const all_examples = [
 {
  label: "Coin exampe",
  value: coin_example,
 },
 {
  label: "Bounded retransmission protocol",
  value: bounded_retransmission,
 },
];
