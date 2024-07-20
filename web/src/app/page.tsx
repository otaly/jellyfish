import { API_URL } from './_lib/config';

type Props = {
  searchParams: {
    name?: string;
  };
};

const fetchGreeting = async (
  params: Props['searchParams'],
): Promise<{ greeting: string }> => {
  const url = new URL('/hello', API_URL);
  if (params.name) url.searchParams.append('name', params.name);

  return (await fetch(url, { cache: 'no-store' })).json();
};

const Home = async ({ searchParams }: Props) => {
  const { greeting } = await fetchGreeting(searchParams);
  return (
    <main className="flex min-h-dvh flex-col items-center p-16">
      <h1>{greeting}</h1>
    </main>
  );
};

export default Home;
