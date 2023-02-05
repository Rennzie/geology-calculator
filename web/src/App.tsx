import './index.css';
import { Canvas, ThreeElements, useFrame } from '@react-three/fiber';
import { useRef, useState } from 'react';
import { Mesh } from 'three';

function Box(props: ThreeElements['mesh']) {
  // This reference will give us direct access to the mesh
  const mesh = useRef<Mesh>(null!);
  // Set up state for the hovered and active state
  const [hovered, setHover] = useState(false);
  const [active, setActive] = useState(false);
  // Subscribe this component to the render-loop, rotate the mesh every frame
  useFrame((state, delta) => {
    // mesh.current.rotation.x += delta;
    mesh.current.rotation.y -= delta / 4;
  });
  // Return view, these are regular three.js elements expressed in JSX
  return (
    <mesh
      {...props}
      ref={mesh}
      // scale={active ? 1.5 : 1}

      onClick={(event) => {
        mesh.current.rotation.x += Math.PI / 2;
        return setActive(!active);
      }}
      onPointerOver={(event) => setHover(true)}
      onPointerOut={(event) => setHover(false)}
      position={[0, 0, 0]}
    >
      <sphereGeometry args={[3, 32, 16, 0, Math.PI * 2, 0, Math.PI / 2]} />
      <meshBasicMaterial wireframe={true} color={'grey'} />
    </mesh>
  );
}

export function App() {
  return (
    <div id="canvas-container">
      <ambientLight args={[0xff0000]} intensity={0.1} />
      <directionalLight position={[0, 0, 5]} intensity={0.5} />
      <Canvas>
        <Box />
      </Canvas>
    </div>
  );
}
