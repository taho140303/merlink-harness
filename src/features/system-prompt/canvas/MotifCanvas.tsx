import { useMemo, useState } from 'react';
import {
  Background,
  Controls,
  MarkerType,
  MiniMap,
  ReactFlow,
  type Edge,
} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import dagre from '@dagrejs/dagre';
import type { Component, EmotionalRange, EventBlock, EventTransition, Motif, StateItem } from '../types';
import { EventBlockNode } from './EventBlockNode';
import { AnimatedFlowEdge } from './AnimatedFlowEdge';
import { EventBlockDetailDialog } from './EventBlockDetailDialog';

const NODE_WIDTH = 220;
const NODE_HEIGHT = 100;

const nodeTypes = { eventBlock: EventBlockNode };
const edgeTypes = { animatedFlow: AnimatedFlowEdge };

interface MotifCanvasProps {
  motif: Motif;
  eventBlocks: EventBlock[];
  eventTransitions: EventTransition[];
  componentById: Map<string, Component>;
  stateById: Map<string, StateItem>;
  emotionsForEventBlock: (ebId: string) => EmotionalRange[];
}

export function MotifCanvas({
  eventBlocks,
  eventTransitions,
  componentById,
  stateById,
  emotionsForEventBlock,
}: MotifCanvasProps) {
  const [selectedEventBlock, setSelectedEventBlock] = useState<EventBlock | null>(null);
  const [dialogOpen, setDialogOpen] = useState(false);

  const eventBlockById = useMemo(
    () => new Map(eventBlocks.map((eb) => [eb.id, eb])),
    [eventBlocks],
  );

  const { nodes, edges } = useMemo(() => {
    const rawNodes = eventBlocks.map((eb) => ({
      id: eb.id,
      type: 'eventBlock' as const,
      position: { x: 0, y: 0 },
      data: {
        eventBlock: eb,
        component: componentById.get(eb.componentId),
        state: stateById.get(eb.stateId),
        onOpenDetail: (target: EventBlock) => {
          setSelectedEventBlock(target);
          setDialogOpen(true);
        },
      },
    }));

    const rawEdges: Edge[] = eventTransitions.map((et) => ({
      id: et.id,
      source: et.fromEventBlockId,
      target: et.toEventBlockId,
      type: 'animatedFlow',
      markerEnd: { type: MarkerType.ArrowClosed, color: 'var(--color-accent)', width: 20, height: 20 },
      data: { transition: et },
    }));

    // Dagre auto-layout (LR)
    const g = new dagre.graphlib.Graph();
    g.setDefaultEdgeLabel(() => ({}));
    g.setGraph({ rankdir: 'LR', nodesep: 60, ranksep: 90 });
    rawNodes.forEach((n) => g.setNode(n.id, { width: NODE_WIDTH, height: NODE_HEIGHT }));
    rawEdges.forEach((e) => g.setEdge(e.source, e.target));
    dagre.layout(g);

    const layoutedNodes = rawNodes.map((n) => {
      const p = g.node(n.id);
      return {
        ...n,
        position: { x: p.x - NODE_WIDTH / 2, y: p.y - NODE_HEIGHT / 2 },
      };
    });

    return { nodes: layoutedNodes, edges: rawEdges };
  }, [eventBlocks, eventTransitions, componentById, stateById]);

  const emotions = selectedEventBlock ? emotionsForEventBlock(selectedEventBlock.id) : [];
  const outgoing = selectedEventBlock
    ? eventTransitions
        .filter((et) => et.fromEventBlockId === selectedEventBlock.id)
        .map((et) => ({ transition: et, target: eventBlockById.get(et.toEventBlockId) }))
    : [];

  return (
    <div className="h-[600px] w-full overflow-hidden rounded-2xl border border-line bg-surface-muted/40  ">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        fitView
        fitViewOptions={{ padding: 0.25 }}
        proOptions={{ hideAttribution: true }}
        nodesDraggable={false}
        nodesConnectable={false}
        elementsSelectable
      >
        <Background gap={20} size={1} color="var(--color-line-strong)" />
        <Controls showInteractive={false} />
        <MiniMap
          pannable
          zoomable
          className="!rounded-lg !border !border-line dark:!border-line"
        />
      </ReactFlow>

      <EventBlockDetailDialog
        open={dialogOpen}
        onOpenChange={setDialogOpen}
        eventBlock={selectedEventBlock}
        component={
          selectedEventBlock
            ? componentById.get(selectedEventBlock.componentId)
            : undefined
        }
        state={selectedEventBlock ? stateById.get(selectedEventBlock.stateId) : undefined}
        emotions={emotions}
        outgoingTransitions={outgoing}
      />
    </div>
  );
}
