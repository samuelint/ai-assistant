import { useConfigurationKV } from '@/lib/configuration/use-configuration-kv';


export function useLLMModel() {
  const { data, mutate, isLoading } = useConfigurationKV('SELECTED_LLM_MODEL');

  return { data: data?.value, mutate, isLoading };
}