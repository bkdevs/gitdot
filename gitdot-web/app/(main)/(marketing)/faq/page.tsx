import { League_Spartan } from "next/font/google";

const league_spartan = League_Spartan({
  subsets: ["latin"],
  weight: ["400", "700"],
});

export default function FAQ() {
  return (
    <div
      className={`${league_spartan.className} blog-root flex flex-col gap-4`}
    >
      <div>
        <p className="font-semibold text-lg">1. What is gitdot?</p>
        <p>
          A better GitHub. <br />
          An opinionated tool for quality open-source software.
        </p>
      </div>
      <div>
        <p className="font-semibold text-lg">2. Who is gitdot for?</p>
        <p>
          Open-source maintainers. <br />
          People who see code as more than a means to an end, but as a craft to
          perfect. The software they build serves the world &mdash; but the
          software they use doesn&apos;t serve them.
        </p>
      </div>
      <div>
        <p className="font-semibold text-lg">
          3. What problem does gitdot solve?
        </p>
        <p>
          A monopoly. <br />
          Open-source software only has one competitive platform: GitHub. And
          while GitHub <i>is</i> an impressive product, we also know that a lack
          of competition enables degradation over time. There&apos;s a few pain
          points we&apos;re keenly aware of (e.g., CI) and make it our mission
          to build a better open-source alternative.
        </p>
      </div>
      <div>
        <p className="font-semibold text-lg">
          4. What features will gitdot have?
        </p>
        <ul className="mb-2">
          <li>• A hyper-performant Git server written in Rust.</li>
          <li>
            • A sane CI/CD platform that is secure by design and locally
            testable.
          </li>
          <li>
            • An issue tracker designed to serve the maintainer, not the
            submitter.
          </li>
        </ul>
        <p>
          We will not have feature parity, but from the get go, our product will
          be reliable. <br />
          It will stink of quality &mdash; and deliver a superior experience for
          a handful of customers.
        </p>
      </div>
      <div>
        <p className="font-semibold text-lg">
          5. What features will gitdot not have?
        </p>
        <ul className="mb-2">
          <li>• No AI copilot.</li>
          <li>• No vanity stars.</li>
          <li>• No free private repos.</li>
        </ul>
        <p>
          We view AI as an implementation detail, not as a feature. We also
          question some of the paradigms present in open-source and ask whether
          features like stars truly serve the maintainer. And finally, public
          repos will be free, but private repos will be paid for.
        </p>
      </div>
      <div>
        <p className="font-semibold text-lg">6. When will gitdot be ready?</p>
        <p>
          Jun 1st, 2026. <br />
          Every week, we will publish a developer log to detail not only our
          progress, but our thinking in full. These will be strikingly
          forthright; we want the <i>why</i> behind our product decisions to be
          critiqued and understood.
        </p>
      </div>

      <p>
        We recognize that we&apos;re making some bold claims here and we&apos;re
        not so naive as to think this will be easy. Building software is hard
        &mdash; but it is simply what we love doing.
      </p>
      <p>&mdash;baepaul & mikkelk.</p>
    </div>
  );
}
