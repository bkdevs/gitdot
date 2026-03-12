const emailTester =
  /^[-!#$%&'*+/0-9=?A-Z^_a-z`{|}~](\.?[-!#$%&'*+/0-9=?A-Z^_a-z`{|}~])*@[a-zA-Z0-9](-*\.?[a-zA-Z0-9])*\.[a-zA-Z](-?[a-zA-Z0-9])+$/;

export function validateRepoSlug(slug: string): boolean {
  return /^[a-zA-Z0-9_-]+$/.test(slug);
}

export function validateEmail(email: string): boolean {
  if (!email) {
    return false;
  }

  const emailParts = email.split("@");
  if (emailParts.length !== 2) {
    return false;
  }

  const account = emailParts[0];
  const address = emailParts[1];
  if (account.length > 64) {
    return false;
  } else if (address.length > 255) {
    return false;
  }

  const domainParts = address.split(".");
  if (domainParts.some((part) => part.length > 63)) {
    return false;
  }
  return emailTester.test(email);
}

export function validatePassword(password: string): boolean {
  return !!password && password.length >= 8;
}
