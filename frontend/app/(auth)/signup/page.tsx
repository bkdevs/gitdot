import SignupForm from "../ui/signup-form";

export default function Page() {
  return (
    <div className="max-w-3xl mx-auto flex items-center justify-center h-screen">
      <SignupForm redirect={"/signup/success"} />
    </div>
  );
}
