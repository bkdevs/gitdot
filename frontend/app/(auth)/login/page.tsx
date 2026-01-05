import LoginForm from "../ui/login-form";

// if already logged in, should redirect back to home?
export default function LoginPage() {
  return (
    <div className="max-w-3xl mx-auto flex gap-4 items-center justify-center h-screen">
      <LoginForm />
    </div>
  );
}
