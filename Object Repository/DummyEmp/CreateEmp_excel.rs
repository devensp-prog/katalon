<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateEmp_excel</name>
   <tag></tag>
   <elementGuidId>35c7fb98-c2b4-4667-a2f7-92ad4f233aa0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;name\&quot;:\&quot;${EmpName}\&quot;,\&quot;salary\&quot;:\&quot;${salary}\&quot;,\&quot;age\&quot;:\&quot;${age}\&quot;}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://dummy.restapiexample.com/api/v1/create</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>95a34d61-cd34-49b3-8660-881b2e6c7222</id>
      <masked>false</masked>
      <name>EmpName</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4f3eeb76-c747-49a9-94a7-46d551d08bdf</id>
      <masked>false</masked>
      <name>salary</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>b1fbc759-e396-4aae-983b-4b7a0b4402b2</id>
      <masked>false</masked>
      <name>age</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
assertThat(response.responseBodyContent.contains(GlobalVariable.EmpName))



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
