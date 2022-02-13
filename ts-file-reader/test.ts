type TEST_TYPE = 'one' | 'two' | 'three';

export interface ITestInterface {
  one: boolean;
  two: string | number;
  three: () => void;
  four: TEST_TYPE;
}

async function main(): Promise<void> {
  const log: string = 'hello';
  console.log(`${log} world`);
}
main();
