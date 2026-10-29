import type { Node } from '@xyflow/react';
import type { Component, EventBlock, StateItem } from '../types';

export type EventBlockNodeData = {
  eventBlock: EventBlock;
  component?: Component;
  state?: StateItem;
  onOpenDetail: (eventBlock: EventBlock) => void;
};

export type EventBlockNodeType = Node<EventBlockNodeData, 'eventBlock'>;
