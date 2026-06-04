import Link from "@/ui/link";
import { leagueSpartan } from "../fonts";

export default function FAQ() {
  return (
    <div
      className={`${leagueSpartan.className} flex flex-col gap-6 px-3 py-4.5 h-full overflow-y-auto scrollbar-none select-none outline-none`}
    >
      <div>
        <p className="font-bold text-lg">1. What is gitdot?</p>
        <p>
          A better GitHub. <br />
          A home for great open-source software.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">2. Who is gitdot for?</p>
        <p>
          Developers. <br />
          People who care. People who see code as more than a means to an end, but as a craft to perfect. <br />
          People who build software <i>well</i> — not because it is the optimal thing to do, but because it is the <i>right</i> thing to do.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">
          3. What problem does gitdot solve?
        </p>
        <p>
          Quality. <br />
          Open-source software only has one competitive platform: GitHub. And while GitHub <i>is</i> an impressive product, we also know that a lack of competition enables degradation over time. There’s a few pain points we’re keenly aware of (e.g., CI) and make it our mission to build a better alternative.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">
          4. What features will gitdot have?
        </p>
        <ul className="mb-2">
          <li>• A hyper-performant Git server written in Rust.</li>
          <li>
            • A code review tool that uses stacked diffs as primitive
          </li>
          <li>
            • A sane CI/CD platform that is secure by design, locally testable and reproducible
          </li>
        </ul>
        <p>
          We think things could be a lot better — and we want to release features that we’re proud of. <br/>
          That does mean that things will take time, as building software right is still hard, but we do think you’ll find it worth it.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">
          5. What features will gitdot not have?
        </p>
        <p>
          AI. <br />
          We view AI as an implementation detail — and do not think that using it is necessarily good. <br />
          In fact, we think it makes many products worse by acting as a bandaid for poor design. <br />
          That isn’t to say we are blind to it, but that we will be judicious in our use of it instead.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">6. When will gitdot have XYZ?</p>
        <p>
          We publicize our roadmap in{" "}
          <Link href="/releases" target="_blank" rel="noopener noreferrer" className="underline">
            /releases
          </Link>{" "}
          with estimated dates for each. <br/>
          While we do think that stands to be pretty complete, if there is something missing, please do{" "}
          <Link href="mailto:founders@gitdot.io" className="underline">
            let us know
          </Link>.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">7. How does gitdot make money?</p>
        <p>
          We don't. <br />
          We are fortunate enough to have raised a small pre-seed round from investors we are happy to call friends, and also to be at a point in our lives where we are financially independent and in good health. <br /><br />

          But it is our intention to build a business that lasts. <br />
          And we will be unabashed in looking for honest and sustainable ways to profit. <br />
          As of now, all repositories are free, but we do envision charging for private repositories for teams in the future.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">8. Does gitdot sell or train on my data?</p>
        <p>
          We do not. <br />

          There are fewer things lamer than selling data for profit.  <br/>
          Your code is your own and there should not be a damn thing to say otherwise. <br/> <br/>
          We make this promise in our{" "}
          <Link href="/privacy" target="_blank" rel="noopener noreferrer" className="underline">
            privacy policy
          </Link>, but also know that even the law is not enough in matters of sovereignty, so plan to build an end-to-end encrypted git protocol as well.

        </p>
      </div>
      <div>
        <p className="font-bold text-lg">9. Is gitdot open source?</p>
        <p>
          Yes. gitdot uses the Apache License. <br/>
          We are committed to open-source, but also do acknowledge the fear of a rug pull or a license change in the future. We do not think we can assuage that anxiety with our words alone: trust is earned, it is not given. <br /><br />

          But we do ask that you hold us accountable: to critique us if we misstep and to fork us if you must.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">10. How was gitdot built?</p>
        <p>
          We document major design decisions in the{" "}
          <Link href="/designs" target="_blank" rel="noopener noreferrer" className="underline">
            /designs
          </Link>{" "}
          folder. <br/>
          We chronicle our progress and our thinking in the{" "}
          <Link href="/weeks" target="_blank" rel="noopener noreferrer" className="underline">
            /weeks
          </Link>{" "}
          folder. <br/><br/>

          As engineers, we are painfully aware of our own inadequacies. <br />
          Building gitdot has made it obviously clear that there is much to learn and to improve still. <br />
          Yet, it is our hope that in some small way, these docs are our opportunity to give back.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">11. Who built gitdot?</p>
        <p>
          <Link href="/bkdevs" target="_blank" rel="noopener noreferrer" className="underline">
            Two dudes
          </Link>{" "}
          in Brooklyn.
        </p>
      </div>
      <div>
        <p className="font-bold text-lg">12. Why is it named gitdot?</p>
        <p>
          <Link href="https://www.youtube.com/watch?v=8aShfolR6w8" target="_blank" rel="noopener noreferrer">
            🐐
          </Link>
        </p>
      </div>
    </div>
  );
}
