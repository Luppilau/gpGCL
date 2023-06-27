const coin_example = `while (x < 0.5) {
  x := uniform(0,1)
}`;

const equal_prob_grid_family = `while(a<=N && b<=N && goal==0){
  if(b == N){
      {goal := 1}[0.5]{goal := 2}
  }else{
      if(a == N){
          a := a - 1
      } else {
          {a := a + 1}[0.5]{b := b + 1}
      }
  }
}`;

const cowboy_duel = `while(0 < flag){
  {flag:=0}[0.75]{
     {flag:=0}[0.33]{flag:=1}
  }
}`;

const sum_01 = `while(0 < n){
  {x:=x+n}[0.5]{skip};
  n := n - 1
}`;

export const all_examples = [
 {
  label: "Coin exampe",
  value: coin_example,
  args:
   '--invarianttype past --templaterefiner inductivity --distance 1 --initialstates "[x=0]"',
 },
 {
  label: "TACAS23: Equal probability grid family",
  value: equal_prob_grid_family,
  args:
   '--post "[goal=1] + [not (goal=1)]*0" --prop "[a<=0 & b<=0 & goal <=0 ]*0.6" --fastparse',
 },
 {
  label: "TACAS23_ABSYNTH: Cowboy duel",
  value: cowboy_duel,
  args:
   '--invarianttype past --templaterefiner inductivity --distance 1 --initialstates "[0<flag]"',
 },
 {
  label: "TACAS23_EXIST: Sum 01",
  value: sum_01,
  args: '--post "x" --prop "[0<n]*(x+0.5*n*0.5)+[not (0<n)]*x"',
 },
];
