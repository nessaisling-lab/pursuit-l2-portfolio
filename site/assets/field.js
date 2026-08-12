/* A domain-warped noise field in the page's own palette, nudged by the pointer.
   Raw WebGL rather than a 3D library: the whole effect is one fragment shader, so
   pulling in Three.js would cost ~600 KB to draw a rectangle. Degrades to the flat
   background if WebGL is missing, and is removed entirely under
   prefers-reduced-motion. */
(function(){
  var reduce = window.matchMedia && matchMedia('(prefers-reduced-motion: reduce)').matches;
  var cv = document.getElementById('field');
  if (!cv || reduce) return;
  var gl = cv.getContext('webgl', {antialias:false, alpha:false, powerPreference:'low-power'});
  if (!gl) return;

  var VS = 'attribute vec2 a;void main(){gl_Position=vec4(a,0.,1.);}';
  var FS = [
  'precision mediump float;',
  'uniform vec2 u_res; uniform float u_t; uniform vec2 u_m;',
  'float hash(vec2 p){return fract(sin(dot(p,vec2(127.1,311.7)))*43758.5453);}',
  'float noise(vec2 p){vec2 i=floor(p),f=fract(p);f=f*f*(3.-2.*f);',
  ' return mix(mix(hash(i),hash(i+vec2(1.,0.)),f.x),mix(hash(i+vec2(0.,1.)),hash(i+vec2(1.,1.)),f.x),f.y);}',
  'float fbm(vec2 p){float v=0.,a=.5;for(int i=0;i<5;i++){v+=a*noise(p);p*=2.02;a*=.5;}return v;}',
  'void main(){',
  ' vec2 uv=gl_FragCoord.xy/u_res;',
  ' float ar=u_res.x/u_res.y;',
  ' vec2 p=vec2(uv.x*ar,uv.y)*2.6;',
  ' vec2 m=(u_m-.5)*1.1;',
  ' float t=u_t*.035;',
  ' vec2 q=vec2(fbm(p+t+m),fbm(p+vec2(5.2,1.3)-t*.8+m*.8));',
  ' float f=fbm(p+q*1.7+t*.25);',
  ' vec3 bg   =vec3(.055,.055,.063);',
  ' vec3 brass=vec3(.722,.537,.235);',
  ' vec3 grn  =vec3(.114,.416,.325);',
  ' vec3 verm =vec3(.808,.259,.169);',
  ' vec3 col=mix(bg,brass,smoothstep(.42,.86,f)*.20);',
  ' col=mix(col,grn,smoothstep(.30,.70,f)*.10);',
  ' col=mix(col,verm,smoothstep(.72,.98,f)*.10);',
  ' float d=distance(uv,vec2(.5,.55));',
  ' col=mix(col,bg,smoothstep(.24,.82,d));',
  ' gl_FragColor=vec4(col,1.);',
  '}'].join('\n');

  function sh(type,src){var o=gl.createShader(type);gl.shaderSource(o,src);gl.compileShader(o);
    if(!gl.getShaderParameter(o,gl.COMPILE_STATUS)){console.warn(gl.getShaderInfoLog(o));return null;}return o;}
  var vs=sh(gl.VERTEX_SHADER,VS), fs=sh(gl.FRAGMENT_SHADER,FS);
  if(!vs||!fs) return;
  var pr=gl.createProgram();gl.attachShader(pr,vs);gl.attachShader(pr,fs);gl.linkProgram(pr);
  if(!gl.getProgramParameter(pr,gl.LINK_STATUS)) return;
  gl.useProgram(pr);

  var buf=gl.createBuffer();gl.bindBuffer(gl.ARRAY_BUFFER,buf);
  gl.bufferData(gl.ARRAY_BUFFER,new Float32Array([-1,-1,3,-1,-1,3]),gl.STATIC_DRAW);
  var a=gl.getAttribLocation(pr,'a');gl.enableVertexAttribArray(a);gl.vertexAttribPointer(a,2,gl.FLOAT,false,0,0);
  var uRes=gl.getUniformLocation(pr,'u_res'),uT=gl.getUniformLocation(pr,'u_t'),uM=gl.getUniformLocation(pr,'u_m');

  /* Half-resolution buffer: the field is deliberately soft, so nobody can see the
     difference and phones keep their battery. */
  function size(){
    var s=Math.min(window.devicePixelRatio||1,1.5)*.5;
    cv.width=Math.max(1,Math.floor(innerWidth*s));
    cv.height=Math.max(1,Math.floor(innerHeight*s));
    gl.viewport(0,0,cv.width,cv.height);
  }
  size(); addEventListener('resize',size,{passive:true});

  var mx=.5,my=.5,tx=.5,ty=.5;
  addEventListener('pointermove',function(e){tx=e.clientX/innerWidth;ty=1-e.clientY/innerHeight;},{passive:true});

  var start=performance.now(), running=true;
  document.addEventListener('visibilitychange',function(){running=!document.hidden;if(running)raf();});
  function frame(now){
    if(!running) return;
    mx+=(tx-mx)*.045; my+=(ty-my)*.045;          /* easing, so it trails the pointer */
    gl.uniform2f(uRes,cv.width,cv.height);
    gl.uniform1f(uT,(now-start)/1000);
    gl.uniform2f(uM,mx,my);
    gl.drawArrays(gl.TRIANGLES,0,3);
    requestAnimationFrame(frame);
  }
  function raf(){requestAnimationFrame(frame);}
  raf(); requestAnimationFrame(function(){cv.classList.add('on');});
})();