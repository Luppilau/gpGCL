export const all_examples = [
 {
  label: "TACAS23: geo",
  value: `while(c <= 0){
  {c := 1}[0.5]{x := x + 1}
}`,
  args: '--post "x" --prop "[c<=0 & x<=0]*(1)"',
 },

 {
  label: "TACAS23: k_geo",
  value: `while(k<=N){
  {k := k + 1; y := y + x; x := 0}[0.5]{x := x + 1}
}`,
  args: '--post "y" --prop "[k<=0 & x<=0 & y<=0]*(N+1)"',
 },

 {
  label: "TACAS23: equal_prob_grid_family (TO)",
  value: `while(a<=N && b<=N && goal==0){
  if(b == N){
      {goal := 1}[0.5]{goal := 2}
  }else{
      if(a == N){
          a := a - 1
      } else {
          {a := a + 1}[0.5]{b := b + 1}
      }
  }
}`,
  args:
   '--post "[goal=1] + [not (goal=1)]*0" --prop "[a<=0 & b<=0 & goal <=0 ]*0.6"',
 },

 {
  label: "TACAS23_ABYSNTH: rfind_mc",
  value: `while(i < k && 0 < flag){
  {flag := 0}[0.5]{flag := 1};
  i := i + 1
}`,
  args:
   '--invarianttype past --templaterefiner inductivity --distance 1 --initialstates "[i<k & 0 < flag]"',
 },

 {
  label: "TACAS23_ABSYNTH: cowboy_duel",
  value: `while(0 < flag){
  {flag:=0}[0.75]{
      {flag:=0}[0.33]{flag:=1}
  }
}`,
  args:
   '--invarianttype past --templaterefiner inductivity --distance 1 --initialstates "[0<flag]"',
 },

 {
  label: "TACAS23_ABSYNTH: filling_vol",
  value: `while(x == 0){
    {x := 0}[0.5]
    {
      {x := 1}[0.5]{x := 2}
    }
  }`,
  args: '--post "[x=2] + [not (x=2)]*0" --prop "[x=2]"',
 },

 {
  label: "TACAS23_EXIST: Bin01_0 (TO)",
  value: `while(0 < n){
  {x := x + y}[0.5]{skip};
  n := n - 1
}`,
  args: '--post "x" --prop "x"',
 },
 {
  label: "TACAS23_EXIST: Geo11_0 (ERR)",
  value: `while(flip == 0){
  {flip := 1}[0.5]{x := 2 * x; z := z + 1}
}`,
  args: '--post "z" --prop "z"',
 },
];
