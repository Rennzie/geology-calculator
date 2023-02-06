import './index.css';

import { Canvas, ThreeElements, useFrame } from '@react-three/fiber';
import { useEffect, useRef, useState } from 'react';
import { DoubleSide, Mesh } from 'three';
import { CameraControls } from '@react-three/drei';

function HalfSphere(props: ThreeElements['mesh']) {
  // This reference will give us direct access to the mesh
  const mesh = useRef<Mesh>(null!);
  // Set up state for the hovered and active state
  const [hovered, setHover] = useState(false);
  useEffect(() => {
    mesh.current.rotation.x += (Math.PI / 2) * 3;
  }, []);
  // Subscribe this component to the render-loop, rotate the mesh every frame
  // useFrame((state, delta) => {
  //   // mesh.current.rotation.x += delta;
  //   mesh.current.rotation.y -= delta / 4;
  // });
  // Return view, these are regular three.js elements expressed in JSX
  return (
    <mesh
      {...props}
      ref={mesh}
      onPointerOver={(event) => setHover(true)}
      onPointerOut={(event) => setHover(false)}
      position={[0, 0, 0]}
    >
      <sphereGeometry args={[3, 32, 16, 0, Math.PI * 2, 0, Math.PI / 2]} />
      <meshBasicMaterial wireframe={true} color={'grey'} />
    </mesh>
  );
}

function Plane(props: ThreeElements['mesh']) {
  // This reference will give us direct access to the mesh
  const mesh = useRef<Mesh>(null!);
  // Set up state for the hovered and active state
  const [hovered, setHover] = useState(false);
  // Subscribe this component to the render-loop, rotate the mesh every frame
  // useFrame((state, delta) => {
  //   // mesh.current.rotation.x += delta;
  //   mesh.current.rotation.y -= delta / 4;
  // });
  // Return view, these are regular three.js elements expressed in JSX
  return (
    <mesh
      {...props}
      ref={mesh}
      onPointerOver={(event) => setHover(true)}
      onPointerOut={(event) => setHover(false)}
      position={[0, 0, -2]}
    >
      <planeGeometry args={[9, 9]} />
      <meshBasicMaterial color={'red'} side={DoubleSide} />
    </mesh>
  );
}

export function App() {
  return (
    <div id="canvas-container">
      <ambientLight args={[0xff0000]} intensity={0.1} />
      <directionalLight position={[0, 0, 5]} intensity={0.5} />
      <Canvas>
        <CameraControls />
        <HalfSphere />
        <Plane />
      </Canvas>
    </div>
  );
}
