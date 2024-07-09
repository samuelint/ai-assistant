import OpenAI from 'openai';

import { useCallback, useEffect, useRef, useState } from 'react';
import { useOpenaiClient } from './openai-client';
import { AssistantStream } from 'openai/lib/AssistantStream.mjs';


export type AssistantStatus = 'in_progress' | 'awaiting_message';
export type Message = OpenAI.Beta.Threads.Messages.Message;
export type MessageContent = OpenAI.Beta.Threads.Messages.MessageContent;
export type CreateMessage = OpenAI.Beta.Threads.Messages.MessageCreateParams;
type MessageDelta = OpenAI.Beta.Threads.Messages.MessageDelta;


interface Props {
    assistantId?: string;
    threadId?: string;
    model?: string;
    temperature?: number;
}
export function useOpenAiAssistant({ assistantId = '', threadId: argsThreadId, model = 'openai:gpt-3.5-turbo', temperature }: Props = {}) {
  const [messages, setMessages] = useState<Message[]>([ ]);
  const [input, setInput] = useState('');
  const [threadId, setThreadId] = useState<string | undefined>(argsThreadId);
  const [status, setStatus] = useState<AssistantStatus>('awaiting_message');
  const [error, setError] = useState<undefined | Error>(undefined);
  const streamRef = useRef<AssistantStream | null>(null);
  const abortControlerRef = useRef<AbortController | null>(null);
  const openai = useOpenaiClient();

  const setUnknownError = useCallback((e: unknown) => {
    if (e instanceof Error) setError(e);
    else setError(new Error(`${e}`));
  }, []);

  useEffect(() => {
    setThreadId(argsThreadId);
    if (!argsThreadId) return;

    const fetchMessages = async () => {
      try {
        const newMessages = await openai.beta.threads.messages.list(argsThreadId);
        setMessages(newMessages.data);
      } catch (e) {
        setUnknownError(e);
      }
    };
    fetchMessages();

  }, [openai.beta.threads.messages, argsThreadId, setUnknownError]);

  const handleInputChange = (
    event:
      | React.ChangeEvent<HTMLInputElement>
      | React.ChangeEvent<HTMLTextAreaElement>,
  ) => {
    setInput(event.target.value);
  };

  const append = async (
    message: CreateMessage,
  ) => {
    try {
      setStatus('in_progress');

      let local_threadId = threadId;
      if (!local_threadId) {
        const thread = await openai.beta.threads.create();
        local_threadId = thread.id;
        setThreadId(local_threadId);
      }

      const created_message = await openai.beta.threads.messages.create(
        local_threadId,
        message
      );
      setMessages(messages => [
        ...messages,
        created_message,
      ]);

      abortControlerRef.current = new AbortController();
      const signal = abortControlerRef.current.signal;

      await new Promise<void>((resolve, rejects) => {
        streamRef.current = openai.beta.threads.runs.stream(local_threadId, {
          model,
          assistant_id: assistantId,
          temperature,
        }, { signal })
          .on('messageCreated', (message: Message) => setMessages(messages => [...messages, message]))
          .on('messageDelta', (_delta: MessageDelta, snapshot: Message) => setMessages(messages => {
            return [
              ...messages.slice(0, messages.length - 1),
              snapshot
            ];
          }))
          .on('messageDone', (message: Message) => {
            return [
              ...messages.slice(0, messages.length - 1),
              message
            ];
          })
          .on('error', (error) => rejects(error))
          .on('abort', () => resolve())
          .on('end', () => resolve());
      });

    } catch (e) {
      setUnknownError(e);
      setMessages(messages => {
        return [
          ...messages.slice(0, messages.length - 1),
        ];
      });
    }
    finally {
      setInput('');
      streamRef.current = null;
      setStatus('awaiting_message');
    }
  };

  const abort = useCallback(() => {
    if (abortControlerRef.current) {
      abortControlerRef.current.abort();
      abortControlerRef.current = null;
    }
  }, []);

  const submitMessage = async (
    event?: React.FormEvent<HTMLFormElement>,
  ) => {
    event?.preventDefault?.();

    if (input === '') {
      return;
    }

    append({ role: 'user', content: input });
  };

  return { input, setInput, messages, setMessages, threadId, error, status, submitMessage, handleInputChange, append, abort };
}