let { initialize } = await import("zokrates-js");
import { readFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

console.log("hello")
initialize().then(async(zokratesProvider) => {
    const filePath = join(__dirname, '../circuits', 'location.zok');
    console.log(filePath);
    const source = await readFile(filePath, 'utf-8');
  
    const artifacts = zokratesProvider.compile(source);
    const { witness, output } = zokratesProvider.computeWitness(artifacts, ["19241908", "73134926", "0", "19241948", "73134823", "0", "10"]);

    const keypair = zokratesProvider.setup(artifacts.program);

    const proof = zokratesProvider.generateProof(
      artifacts.program,
      witness,
      keypair.pk
    );
    console.log("proof", proof);
    const verifier = zokratesProvider.exportSolidityVerifier(keypair.vk);

    const isVerified = zokratesProvider.verify(keypair.vk, proof);
    console.log("isVerified", isVerified);
});
  